# steve-jobs design documentation

### Aim and scope:

I will be using Rust in an attempt to meet the requirements specified by level 5 of your challenge. Furthermore, the application will only accomodate linux distributions as attempting something like this for Windows will significantly increase the complexity of this challenge and I may fail to complete the challenge in the allotted time.

### Bells and whistles:

The application will be structured into a simple client and server architecture. There will be 3 modules lazily named as "auth", "client" and "server" that will make up the application's cargo package.

The auth module will predictively take care of the authorization for the application.

The server will expose the following endpoints for the clients to interact with:

1) Server: To start the gRPC server.
2) Shell: To start a unix process and return a JobId as an output.
3) Stop: To kill the process associated with the JobId provided as an input.
4) Status: To get the status of the process associated with the JobId provided as an input.
5) Result: To get the output the process associated with the JobId provided as an input.
6) Streamer: To stream the output of the process associated with the JobId provided as an input.

The client module will offer subcommands that correspond to each of the server's endpoints.

Example UX: mrarsenal@pop-os:~/Documents/steve-jobs$./target/debug/steve-jobs run "echo hi"

### Some specifics:

1) I plan to get the username from the shell calling the client subcommand. And only a user named "mrarsenal" (huge fan of ArsenalFC) will be authorized to avail the services of the application.
2) The authorization will be done using JWT. The secret key "secret" will be hardcoded in the code and will be used to evaluate the client's claims.
3) Then mTLS will use AES256-SHA256 bulk encryption to secure the communications between the server and the client.
4) The client and server certificates will be pregenerated and will be available as a part of the repository.
5) The client and server will communicate using the gRPC protocol (enforced by the specification).
6) In order to do resource throttling for the job I intend to use the cgroup functionality exposed by the linux kernel.
7) In order to isolate jobs I intend to use the unshare system call offered by the linux API.

#### Configuring mTLS:
1) I plan to generate a self signed root CA certificate.
2) Then I will generate a key-certificate pair for the client and server.
3) I will make sure they have different common names and serial numbers.
4) Then I will use the CA key to sign the client and server certificates with a certificate signing request.
5) Needless to say the communication will be encrypted.


### Limitations/Trade-Offs/Ruthlesly-Cutting-Corners:
** This is only due to time constraints and I may end up addressing some of these issues. **

1) I will assume that a intelligent user is interacting with the application. That means that he won't try to fetch the output of a process in the running state. He won't kill or try to fetch the output of a non existent JobId. He won't make mistakes entering the command and the arguments. He won't try to get the output of a killed JobId. He won't try to call the client with a username that is not "mrarsenal".
2) Output will not be persistent. That is, it can only be fetched from the "Result" endpoint once. Atfer that it's status won't be available on the "Status" endpoint.
3) Also, the output of a killed job will not be available.
4) There will be no provision to get the status and output of a streamed job because it doesn't make sense to me. You will be seeing the output live. Also, no JobId will be generated for the streamed job.
5) I have decided to go with JWT because it's battle tested for me in Rust.
6) I will not be demonstrating resource throttling and process isolation on the streamed job as I think that doing so for the job spawned by the "Run" endpoint will be enough to demonstrate that I am familiar with resource throttling and process isolation as a proof of concept.
7) Error handling may not be exhaustive.
8) Output of a job will be everything that it prints on stdout.
9) Out will be raw gRPC messages and not pretty-printed strings.
10) Test coverage may not be exhaustive.


### Future enhancements:

1) Tracing.
2) Execution of piped processes.
3) Pausable/Resumable jobs.
4) Health checks.
5) Sessions.
