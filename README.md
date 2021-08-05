Read Challenge.pdf first for more context.

# steve-jobs design documentation

### Aim and scope:

I will be using Rust in an attempt to meet the requirements specified by level 5 of your challenge. Furthermore, the application will only accomodate linux distributions as attempting something like this for Windows will significantly increase the complexity of this challenge and I may fail to complete the challenge in the allotted time.

### Bells and whistles:

The application will be structured into a simple client and server architecture. There will be 3 modules lazily named as "auth", "client" and "server" that will make up the application's cargo package.

The auth module will predictively take care of the authorization for the application.

The server will expose the following endpoints for the clients to interact with:

1. Server: To start the gRPC server.
2. Shell: To start a unix process and return a JobId as an output.
3. Stop: To kill the process associated with the JobId provided as an input.
4. Status: To get the status of the process associated with the JobId provided as an input.
5. Result: To get the output the process associated with the JobId provided as an input.
6. Streamer: To stream the output of the process associated with the JobId provided as an input.

The client module will offer subcommands that correspond to each of the server's endpoints.

### Some specifics:

1. I plan to get the username from the shell calling the client subcommand. And only a user named "mrarsenal" (huge fan of ArsenalFC) will be authorized to avail the services of the application.
2. The authorization will be done using JWT. The secret key "secret" will be hardcoded in the code and will be used to evaluate the client's claims.
3. The client and server certificates will be pregenerated and will be available as a part of the repository.
4. In order to do resource throttling for the job I intend to use the cgroup functionality exposed by the linux kernel.
5. In order to isolate jobs I intend to use the unshare system call offered by the linux API.

### Usage:

1. cargo run -- server
2. cargo run -- run "ls -al"
3. cargo run -- stop UUID
4. cargo run -- status UUID
5. cargo run -- result UUID
6. cargo run -- streamer "ls -al"
