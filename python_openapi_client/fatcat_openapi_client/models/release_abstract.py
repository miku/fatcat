# coding: utf-8

"""
    fatcat

    Fatcat is a scalable, versioned, API-oriented catalog of bibliographic entities and file metadata.   # noqa: E501

    The version of the OpenAPI document: 0.3.1
    Contact: webservices@archive.org
    Generated by: https://openapi-generator.tech
"""


import pprint
import re  # noqa: F401

import six


class ReleaseAbstract(object):
    """NOTE: This class is auto generated by OpenAPI Generator.
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    """
    Attributes:
      openapi_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    openapi_types = {
        'sha1': 'str',
        'content': 'str',
        'mimetype': 'str',
        'lang': 'str'
    }

    attribute_map = {
        'sha1': 'sha1',
        'content': 'content',
        'mimetype': 'mimetype',
        'lang': 'lang'
    }

    def __init__(self, sha1=None, content=None, mimetype=None, lang=None):  # noqa: E501
        """ReleaseAbstract - a model defined in OpenAPI"""  # noqa: E501

        self._sha1 = None
        self._content = None
        self._mimetype = None
        self._lang = None
        self.discriminator = None

        if sha1 is not None:
            self.sha1 = sha1
        if content is not None:
            self.content = content
        if mimetype is not None:
            self.mimetype = mimetype
        if lang is not None:
            self.lang = lang

    @property
    def sha1(self):
        """Gets the sha1 of this ReleaseAbstract.  # noqa: E501

        SHA-1 hash of data, in hex encoding  # noqa: E501

        :return: The sha1 of this ReleaseAbstract.  # noqa: E501
        :rtype: str
        """
        return self._sha1

    @sha1.setter
    def sha1(self, sha1):
        """Sets the sha1 of this ReleaseAbstract.

        SHA-1 hash of data, in hex encoding  # noqa: E501

        :param sha1: The sha1 of this ReleaseAbstract.  # noqa: E501
        :type: str
        """
        if sha1 is not None and len(sha1) > 40:
            raise ValueError("Invalid value for `sha1`, length must be less than or equal to `40`")  # noqa: E501
        if sha1 is not None and len(sha1) < 40:
            raise ValueError("Invalid value for `sha1`, length must be greater than or equal to `40`")  # noqa: E501
        if sha1 is not None and not re.search(r'[a-f0-9]{40}', sha1):  # noqa: E501
            raise ValueError(r"Invalid value for `sha1`, must be a follow pattern or equal to `/[a-f0-9]{40}/`")  # noqa: E501

        self._sha1 = sha1

    @property
    def content(self):
        """Gets the content of this ReleaseAbstract.  # noqa: E501

        Abstract content. May be encoded, as per `mimetype` field, but only string/text content may be included.   # noqa: E501

        :return: The content of this ReleaseAbstract.  # noqa: E501
        :rtype: str
        """
        return self._content

    @content.setter
    def content(self, content):
        """Sets the content of this ReleaseAbstract.

        Abstract content. May be encoded, as per `mimetype` field, but only string/text content may be included.   # noqa: E501

        :param content: The content of this ReleaseAbstract.  # noqa: E501
        :type: str
        """

        self._content = content

    @property
    def mimetype(self):
        """Gets the mimetype of this ReleaseAbstract.  # noqa: E501

        Mimetype of abstract contents. `text/plain` is the default if content isn't encoded.   # noqa: E501

        :return: The mimetype of this ReleaseAbstract.  # noqa: E501
        :rtype: str
        """
        return self._mimetype

    @mimetype.setter
    def mimetype(self, mimetype):
        """Sets the mimetype of this ReleaseAbstract.

        Mimetype of abstract contents. `text/plain` is the default if content isn't encoded.   # noqa: E501

        :param mimetype: The mimetype of this ReleaseAbstract.  # noqa: E501
        :type: str
        """

        self._mimetype = mimetype

    @property
    def lang(self):
        """Gets the lang of this ReleaseAbstract.  # noqa: E501

        ISO language code of the abstract. Same semantics as release `language` field.   # noqa: E501

        :return: The lang of this ReleaseAbstract.  # noqa: E501
        :rtype: str
        """
        return self._lang

    @lang.setter
    def lang(self, lang):
        """Sets the lang of this ReleaseAbstract.

        ISO language code of the abstract. Same semantics as release `language` field.   # noqa: E501

        :param lang: The lang of this ReleaseAbstract.  # noqa: E501
        :type: str
        """

        self._lang = lang

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.openapi_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, ReleaseAbstract):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
