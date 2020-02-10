Kulvert is an attempt to learn Rust web development by building a
simple analogue to OpenStack Keystone.  As of 7 February, 2020, it
shows how to handle simple URLs.  Here are the additional tasks I
would like to accomplish in the next few iterations.

* Provide a way to load sample data from a file.
* Have a precammed identity provider called "built-in"
* Have a hardcoded protocol called "client-cert".
* Allow a user to log in from sample Data and client certificate
* Once a user is authenticated, set a session cookie.
* A user with a session cookie gets an RBAC driven view of the UI.
