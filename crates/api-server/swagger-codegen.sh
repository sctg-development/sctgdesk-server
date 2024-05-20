#!/bin/bash
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:21114/openapi.json)

if [ "$response" -eq 200 ]; then
    echo "Success! Server is up and running."
    # Continue
    curl http://127.0.0.1:21114/openapi.json >webconsole/src/api/openapi.json &&
        docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli-v3 generate -i file:///local/webconsole/src/api/openapi.json -l typescript-axios -o /local/webconsole/src/api &&
        sed -ibak 's/accessToken/access_token/' webconsole/src/api/models/login-reply.ts &&
        sed -ibak 's/isAdmin/is_admin/' webconsole/src/api/models/user-list-response.ts &&
        sed -ibak 's/lastOnline/last_online/' webconsole/src/api/models/peer.ts &&
        sed -ibak -e 's/confirmPassword/\"confirm-password\"/' -e 's/isAdmin/is_admin/' -e 's/groupName/group_name/' webconsole/src/api/models/add-user-request.ts &&
        rm webconsole/src/api/models/*tsbak
else
    echo "Error: Server is not responding. you must start sctgdesk-api-server first."
    exit 1
fi
