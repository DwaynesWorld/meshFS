### API

- GET v1/blobs

  - 200: list blobs.

- GET v1/blobs/key

  - 302: redirect to volume server.
  - 404: key not found

- PUT v1/blobs/key

  - 201: written
  - 404: key not found

- DELETE v1/blobs/key

  - 204: deleted
  - 404: key not found

### Usage

```
# put "bar" in key "foo" (will 403 if it already exists)
curl -v -L -X PUT -d bar localhost:3000/v1/blobs/foo

# get key "foo" (should be "bar")
curl -v -L localhost:3000/v1/blobs/foo

# get key list
curl -v -L localhost:3000/v1/blobs

# delete key "test_key"
curl -v -L -X DELETE localhost:3000/v1/blobs/test_key

---

# put file in key "file.txt"
curl -v -L -X PUT -T README.md localhost:3000/v1/blobs/fizz

# get file in key "file.txt"
curl -v -L -o README-1.md localhost:3000/v1/blobs/fizz

# delete key "fizz"
curl -v -L -X DELETE localhost:3000/v1/blobs/fizz
```
