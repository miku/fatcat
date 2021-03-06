
# Fatcat Production Import CHANGELOG

This file tracks major content (metadata) imports to the Fatcat production
database (at https://fatcat.wiki). It complements the code CHANGELOG file.

In general, changes that impact more than 50k entities will get logged here;
this file should probably get merged into the guide at some point.

This file should not turn in to a TODO list!

## 2019-10

Updated 304,308 file entities to remove broken
"https://web.archive.org/web/None/*" URLs.

## 2019-09

Created and updated metadata for tens of thousands of containers, using
"chocula" pipeline.

## 2019-08

Merged/fixed roughly 100 container entities with invalid ISSN-L numbers (eg,
invalid ISSN checksum).

## 2019-04

Imported files (matched to releases by DOI) from Semantic Scholar
(`DIRECT-OA-CRAWL-2019` crawl).

    Arabesque importer
    crawl-bot
    `s2_doi.sqlite`
    TODO: archive.org link
    TODO: rough count
    TODO: date

Imported files (matched to releases by DOI) from pre-1923/pre-1909 items uploaded
by a user to archive.org.

    Matched importer
    internetarchive-bot (TODO:)
    TODO: archive.org link
    TODO: counts
    TODO: date

Imported files (matched to releases by DOI) from CORE.ac.uk
(`DIRECT-OA-CRAWL-2019` crawl).

Imported files (matched to releases by DOI) from the public web (including many
repositories) from the `UNPAYWALL` 2018 crawl.

## 2019-02

Bootstrapped!
