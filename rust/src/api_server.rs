//! API endpoint handlers

use ConnectionPool;
use chrono;
use database_models::*;
use database_schema::{changelog, container_edit, container_ident, container_rev, creator_edit,
                      creator_ident, creator_rev, editgroup, editor, file_edit, file_ident,
                      file_rev, release_edit, release_ident, release_rev, work_edit, work_ident,
                      work_rev};
use diesel::prelude::*;
use diesel::{self, insert_into};
use errors::*;
use fatcat_api::models;
use fatcat_api::models::*;
use fatcat_api::{Api, ApiError, ContainerIdGetResponse, ContainerLookupGetResponse,
                 ContainerPostResponse, Context, CreatorIdGetResponse, CreatorLookupGetResponse,
                 CreatorPostResponse, EditgroupIdAcceptPostResponse, EditgroupIdGetResponse,
                 EditgroupPostResponse, EditorUsernameChangelogGetResponse,
                 EditorUsernameGetResponse, FileIdGetResponse, FileLookupGetResponse,
                 FilePostResponse, ReleaseIdGetResponse, ReleaseLookupGetResponse,
                 ReleasePostResponse, WorkIdGetResponse, WorkPostResponse};
use futures::{self, Future};
use uuid;

// Helper for calling through to handlers
macro_rules! wrap_get_id_handler {
    ($get_fn:ident, $handler:ident, $resp:ident, $idtype:ident) => {
        fn $get_fn(
            &self,
            id: $idtype,
            _context: &Context,
        ) -> Box<Future<Item = $resp, Error = ApiError> + Send> {
            match self.$handler(id) {
                Ok(Some(entity)) =>
                    Box::new(futures::done(Ok($resp::FoundEntity(entity)))),
                Ok(None) =>
                    Box::new(futures::done(Ok($resp::NotFound(
                        ErrorResponse { message: "No such entity".to_string() }),
                    ))),
                Err(e) =>
                    // TODO: dig in to error type here
                    Box::new(futures::done(Ok($resp::BadRequest(
                        ErrorResponse { message: e.to_string() },
                    )))),
            }
        }
    }
}

#[derive(Clone)]
pub struct Server {
    pub db_pool: ConnectionPool,
}

impl Server {
    fn container_id_get_handler(&self, id: String) -> Result<Option<ContainerEntity>> {
        let conn = self.db_pool.get().expect("db_pool error");
        let id = uuid::Uuid::parse_str(&id)?;

        let res: ::std::result::Result<(ContainerIdentRow, ContainerRevRow), _> =
            container_ident::table
                .find(id)
                .inner_join(container_rev::table)
                .first(&conn);

        let (ident, rev) = match res {
            Ok(r) => r,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let entity = ContainerEntity {
            issn: rev.issn,
            publisher: rev.publisher,
            name: rev.name,
            state: None, // TODO:
            ident: Some(ident.id.to_string()),
            revision: ident.rev_id.map(|v| v as isize),
            redirect: ident.redirect_id.map(|u| u.to_string()),
            editgroup: None,
        };
        Ok(Some(entity))
    }

    fn creator_id_get_handler(&self, id: String) -> Result<Option<CreatorEntity>> {
        let conn = self.db_pool.get().expect("db_pool error");
        let id = uuid::Uuid::parse_str(&id)?;

        let res: ::std::result::Result<(CreatorIdentRow, CreatorRevRow), _> = creator_ident::table
            .find(id)
            .inner_join(creator_rev::table)
            .first(&conn);

        let (ident, rev) = match res {
            Ok(r) => r,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let entity = CreatorEntity {
            name: rev.name,
            orcid: rev.orcid,
            state: None, // TODO:
            ident: Some(ident.id.to_string()),
            revision: ident.rev_id.map(|v| v as isize),
            redirect: ident.redirect_id.map(|u| u.to_string()),
            editgroup: None,
        };
        Ok(Some(entity))
    }

    fn file_id_get_handler(&self, id: String) -> Result<Option<FileEntity>> {
        let conn = self.db_pool.get().expect("db_pool error");
        let id = uuid::Uuid::parse_str(&id)?;

        let res: ::std::result::Result<(FileIdentRow, FileRevRow), _> = file_ident::table
            .find(id)
            .inner_join(file_rev::table)
            .first(&conn);

        let (ident, rev) = match res {
            Ok(r) => r,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let entity = FileEntity {
            sha1: rev.sha1,
            size: rev.size.map(|v| v as isize),
            url: rev.url,
            state: None, // TODO:
            ident: Some(ident.id.to_string()),
            revision: ident.rev_id.map(|v| v as isize),
            redirect: ident.redirect_id.map(|u| u.to_string()),
            editgroup: None,
        };
        Ok(Some(entity))
    }

    fn work_id_get_handler(&self, id: String) -> Result<Option<WorkEntity>> {
        let conn = self.db_pool.get().expect("db_pool error");
        let id = uuid::Uuid::parse_str(&id)?;

        let res: ::std::result::Result<(WorkIdentRow, WorkRevRow), _> = work_ident::table
            .find(id)
            .inner_join(work_rev::table)
            .first(&conn);

        let (ident, rev) = match res {
            Ok(r) => r,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let entity = WorkEntity {
            work_type: rev.work_type,
            state: None, // TODO:
            ident: Some(ident.id.to_string()),
            revision: ident.rev_id.map(|v| v as isize),
            redirect: ident.redirect_id.map(|u| u.to_string()),
            editgroup: None,
        };
        Ok(Some(entity))
    }

    fn release_id_get_handler(&self, id: String) -> Result<Option<ReleaseEntity>> {
        let conn = self.db_pool.get().expect("db_pool error");
        let id = uuid::Uuid::parse_str(&id)?;

        let res: ::std::result::Result<(ReleaseIdentRow, ReleaseRevRow), _> = release_ident::table
            .find(id)
            .inner_join(release_rev::table)
            .first(&conn);

        let (ident, rev) = match res {
            Ok(r) => r,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let entity = ReleaseEntity {
            title: rev.title,
            release_type: rev.release_type,
            //date: rev.date,
            doi: rev.doi,
            volume: rev.volume,
            pages: rev.pages,
            issue: rev.issue,
            container_id: None, // TODO
            work_id: None,      // TODO
            state: None,        // TODO:
            ident: Some(ident.id.to_string()),
            revision: ident.rev_id.map(|v| v as isize),
            redirect: ident.redirect_id.map(|u| u.to_string()),
            editgroup: None,
        };
        Ok(Some(entity))
    }

    fn editgroup_id_get_handler(&self, id: i32) -> Result<Option<Editgroup>> {
        let conn = self.db_pool.get().expect("db_pool error");

        let row: EditgroupRow = editgroup::table.find(id as i64).first(&conn)?;

        let eg = Editgroup {
            id: Some(row.id as isize),
            editor_id: row.editor_id as isize,
            description: row.description,
        };
        Ok(Some(eg))
    }

    fn editor_get_handler(&self, username: String) -> Result<Option<Editor>> {
        let conn = self.db_pool.get().expect("db_pool error");

        let row: EditorRow = editor::table
            .filter(editor::username.eq(&username))
            .first(&conn)?;

        let ed = Editor {
            username: row.username,
        };
        Ok(Some(ed))
    }

    fn editor_changelog_get_handler(&self, username: String) -> Result<Option<Changelogentries>> {
        let conn = self.db_pool.get().expect("db_pool error");

        // TODO: single query
        let editor: EditorRow = editor::table
            .filter(editor::username.eq(username))
            .first(&conn)?;
        let changes: Vec<(ChangelogRow, EditgroupRow)> = changelog::table
            .inner_join(editgroup::table)
            .filter(editgroup::editor_id.eq(editor.id))
            .load(&conn)?;

        let entries = changes
            .iter()
            .map(|(row, _)| ChangelogentriesInner {
                index: row.id as isize,
                editgroup_id: row.editgroup_id as isize,
                timestamp: chrono::DateTime::from_utc(row.timestamp, chrono::Utc),
            })
            .collect();
        Ok(Some(entries))
    }
}

impl Api for Server {
    wrap_get_id_handler!(
        container_id_get,
        container_id_get_handler,
        ContainerIdGetResponse,
        String
    );
    wrap_get_id_handler!(
        creator_id_get,
        creator_id_get_handler,
        CreatorIdGetResponse,
        String
    );
    wrap_get_id_handler!(file_id_get, file_id_get_handler, FileIdGetResponse, String);
    wrap_get_id_handler!(work_id_get, work_id_get_handler, WorkIdGetResponse, String);
    wrap_get_id_handler!(
        release_id_get,
        release_id_get_handler,
        ReleaseIdGetResponse,
        String
    );
    wrap_get_id_handler!(
        editgroup_id_get,
        editgroup_id_get_handler,
        EditgroupIdGetResponse,
        i32
    );

    fn container_lookup_get(
        &self,
        issn: String,
        _context: &Context,
    ) -> Box<Future<Item = ContainerLookupGetResponse, Error = ApiError> + Send> {
        let conn = self.db_pool.get().expect("db_pool error");

        let res: ::std::result::Result<(ContainerIdentRow, ContainerRevRow), _> =
            container_ident::table
                .inner_join(container_rev::table)
                .first(&conn);
        // XXX: actually do a filter/lookup

        let (ident, rev) = match res {
            Ok(r) => r,
            Err(_) => {
                return Box::new(futures::done(Ok(ContainerLookupGetResponse::BadRequest(
                    ErrorResponse {
                        message: "No such container".to_string(),
                    },
                ))));
            }
        };

        let entity = ContainerEntity {
            issn: rev.issn,
            publisher: rev.publisher,
            name: rev.name,
            state: None, // TODO:
            ident: Some(ident.id.to_string()),
            revision: ident.rev_id.map(|v| v as isize),
            redirect: ident.redirect_id.map(|u| u.to_string()),
            editgroup: None,
        };
        Box::new(futures::done(Ok(ContainerLookupGetResponse::FoundEntity(
            entity,
        ))))
    }

    fn container_post(
        &self,
        body: models::ContainerEntity,
        _context: &Context,
    ) -> Box<Future<Item = ContainerPostResponse, Error = ApiError> + Send> {
        //let editgroup_id: i64 = body.editgroup.expect("need editgroup_id") as i64;
        // TODO: or find/create
        let editgroup_id = 1;
        let conn = self.db_pool.get().expect("db_pool error");

        let edit: Vec<ContainerEditRow> = diesel::sql_query(
            "WITH rev AS ( INSERT INTO container_rev (name, publisher, issn)
                        VALUES ($1, $2, $3)
                        RETURNING id ),
                ident AS ( INSERT INTO container_ident (rev_id)
                            VALUES ((SELECT rev.id FROM rev))
                            RETURNING id )
            INSERT INTO container_edit (editgroup_id, ident_id, rev_id) VALUES
                ($4, (SELECT ident.id FROM ident), (SELECT rev.id FROM rev))
            RETURNING *",
        ).bind::<diesel::sql_types::Text, _>(body.name)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(body.publisher)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(body.issn)
            .bind::<diesel::sql_types::BigInt, _>(editgroup_id)
            .load(&conn)
            .unwrap();
        let edit = &edit[0];

        let entity_edit = EntityEdit {
            editgroup_id: Some(edit.editgroup_id as isize),
            revision: Some(edit.rev_id.unwrap() as isize),
            ident: Some(edit.ident_id.to_string()),
            edit_id: Some(edit.id as isize),
        };
        Box::new(futures::done(Ok(ContainerPostResponse::CreatedEntity(
            entity_edit,
        ))))
    }

    fn creator_lookup_get(
        &self,
        orcid: String,
        context: &Context,
    ) -> Box<Future<Item = CreatorLookupGetResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "creator_lookup_get(\"{}\") - X-Span-ID: {:?}",
            orcid,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn creator_post(
        &self,
        body: models::CreatorEntity,
        context: &Context,
    ) -> Box<Future<Item = CreatorPostResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "creator_post({:?}) - X-Span-ID: {:?}",
            body,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn file_lookup_get(
        &self,
        sha1: String,
        context: &Context,
    ) -> Box<Future<Item = FileLookupGetResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "file_lookup_get(\"{}\") - X-Span-ID: {:?}",
            sha1,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn file_post(
        &self,
        body: models::FileEntity,
        context: &Context,
    ) -> Box<Future<Item = FilePostResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "file_post({:?}) - X-Span-ID: {:?}",
            body,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn work_post(
        &self,
        body: models::WorkEntity,
        context: &Context,
    ) -> Box<Future<Item = WorkPostResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "work_post({:?}) - X-Span-ID: {:?}",
            body,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn release_lookup_get(
        &self,
        doi: String,
        context: &Context,
    ) -> Box<Future<Item = ReleaseLookupGetResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "release_lookup_get(\"{}\") - X-Span-ID: {:?}",
            doi,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn release_post(
        &self,
        body: models::ReleaseEntity,
        context: &Context,
    ) -> Box<Future<Item = ReleasePostResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "release_post({:?}) - X-Span-ID: {:?}",
            body,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn editgroup_id_accept_post(
        &self,
        id: i32,
        context: &Context,
    ) -> Box<Future<Item = EditgroupIdAcceptPostResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "editgroup_id_accept_post({}) - X-Span-ID: {:?}",
            id,
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn editgroup_post(
        &self,
        context: &Context,
    ) -> Box<Future<Item = EditgroupPostResponse, Error = ApiError> + Send> {
        let context = context.clone();
        println!(
            "editgroup_post() - X-Span-ID: {:?}",
            context.x_span_id.unwrap_or(String::from("<none>")).clone()
        );
        Box::new(futures::failed("Generic failure".into()))
    }

    fn editor_username_changelog_get(
        &self,
        username: String,
        _context: &Context,
    ) -> Box<Future<Item = EditorUsernameChangelogGetResponse, Error = ApiError> + Send> {
        match self.editor_changelog_get_handler(username) {
            Ok(Some(entries)) =>
                Box::new(futures::done(Ok(EditorUsernameChangelogGetResponse::FoundMergedChanges(entries)))),
            Ok(None) =>
                Box::new(futures::done(Ok(EditorUsernameChangelogGetResponse::NotFound(
                    ErrorResponse { message: "No such entity".to_string() }),
                ))),
            Err(e) =>
                // TODO: dig in to error type here
                Box::new(futures::done(Ok(EditorUsernameChangelogGetResponse::GenericError(
                    ErrorResponse { message: e.to_string() },
                )))),
        }
    }

    fn editor_username_get(
        &self,
        username: String,
        _context: &Context,
    ) -> Box<Future<Item = EditorUsernameGetResponse, Error = ApiError> + Send> {
        match self.editor_get_handler(username) {
            Ok(Some(entity)) =>
                Box::new(futures::done(Ok(EditorUsernameGetResponse::FoundEditor(entity)))),
            Ok(None) =>
                Box::new(futures::done(Ok(EditorUsernameGetResponse::NotFound(
                    ErrorResponse { message: "No such entity".to_string() }),
                ))),
            Err(e) =>
                // TODO: dig in to error type here
                Box::new(futures::done(Ok(EditorUsernameGetResponse::GenericError(
                    ErrorResponse { message: e.to_string() },
                )))),
        }
    }
}
