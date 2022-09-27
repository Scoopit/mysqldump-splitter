# mysqldump-splitter

Split gigantic mysql dumps into smaller one in a human manageable directory structure.

Output files can be optionally be compressed (gzip).

Known Issues:

- procedures or triggers are appended to the last table of a database
- comments do not match files (eg. the create database file
  contains comments for the first table in the db)
