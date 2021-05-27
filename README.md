### API

- GET /key

  - 302 redirect to volume server.

- PUT /key

  - 201 = written

- DELETE /key

  - 204 = deleted

### Usage

```
# put "test_value" in key "test_key" (will 403 if it already exists)
curl -v -L -X PUT -d test_value localhost:3000/test_key

# get key "test_key" (should be "test_value")
curl -v -L localhost:3000/test_key

# delete key "test_key"
curl -v -L -X DELETE localhost:3000/test_key

---

# put file in key "file.txt"
curl -v -L -X PUT -T /path/to/local/file.txt localhost:3000/file.txt

# get file in key "file.txt"
curl -v -L -o /path/to/local/file.txt localhost:3000/file.txt

# delete key "file.txt"
curl -v -L -X DELETE localhost:3000/test_key
```
