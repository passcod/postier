# Postier

Only accepts POST requests. Takes the request's path and, if a file
`hooks/$path` (defaulting to `hooks/default` if the path is empty)
exists in the current directory, executes it. It never returns any
content through HTTP, but routes stdout and stderr of child processes
to the server's I/O.

Response status:
- 400: not a POST request
- 404: hook not found
- 204: hook executed

[MIT](http://passcod.mit-license.org/)
