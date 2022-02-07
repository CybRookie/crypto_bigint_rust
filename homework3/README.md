# Homework 3

Table of contents:  

- [Homework 3](#homework-3)
  - [Idea](#idea)
  - [Note](#note)

Task description:  

```txt
Convert the first and the second homeworks to a web application,
implementing Caesar, Vigenere, Diffie-Hellman and RSA.
Save operations to a simple database,
pay attention to the resource ownership!
```

The project is unfinished!  
The Actix framework was used of the backend.  
The Yew framework was used for the frontend WebAssembly SPA webapp.  
SQLite was planned to be used for the authentication, Diffie-Hellman requests history storage and unfinished Diffie-Hellman request.  

## Idea

The idea was to create a WebAssembly SPA webapp, that would dynamically change the rendered page based on the changes to 
its internal state values. When a request from the form for cipher calculations or the signup/login/logout would be made, 
a POST request would be sent to the backend par to the whole application.  

The backed would validate the received values, if they are correct, proceed with the calculations and return a produced value, 
or return an error message. If the request was related to the authentication, then it would be validated as well and 
user information would be stored in the SQLite database, password would be ideally hashed and salted. Authentication would 
be done with the implementation of JWTs.  

## Note

The project is unfinished, while the partial implementations of the frontend and backend can be compiled and used, 
the frontend part is unable to send custom GET/POST fetch requests to the backend asynchronously. 
Custom Rust libraries for asynchronous runtimes do not support WebAssembly target; 
frontend's framework service will be deprecated and complicated webapp's structure; 
creating bindings to the JavaScript's promises is even more complicated, because of the switching of values between
WebAssembly binary and JavaScript runtime and general nature of produced data type, which may lose required methods for 
operation.  

>The project could be finished, if there was more time on the hands. A lot of it was spent on learning the Rust WebAssembly, 
WebAssembly in general, different frameworks for backend and frontend, and on the experiments with them.  
