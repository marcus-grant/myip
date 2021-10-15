# MyIP

A simple rust server intended for serverless environments, like Cloudflare Edge Workers in my case, that will get the public facing IP of the requester's network. It will check the `X-FORWARDED-FOR` HTTP header in the requester's HTTP request to find the address to the requester's netowrk. I wrote it in Rust because it's fast, secure, supported on Cloudflare workers, and of course because I'm learning how to use the language and thought this would be an ideal use case to learn it.


