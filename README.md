# MyIP

A simple rust server intended for serverless environments, like Cloudflare Edge Workers in my case, that will get the public facing IP of the requester's network. It will check the `X-FORWARDED-FOR` HTTP header in the requester's HTTP request to find the address to the requester's netowrk. I wrote it in Rust because it's fast, secure, supported on Cloudflare workers, and of course because I'm learning how to use the language and thought this would be an ideal use case to learn it.


## JS Worker

The first version of this was made using the JS version of Cloudflare's worker API. The native rust implementation is too buggy for me to mess with right now, so might as well use the Javascript API instead. The standalone server with its minimized container will be finished sometime in the future.

To use this server, simply send an HTTP request to wherever this worker is run and in plain text the response will be your public facing IP address. This is done by examining one of three HTTP headers in this order: `X-Forwarded-For`, `X-Real-IP`, `CF-Connecting-IP`. The first one detected is used as a plaintext response containing your public IP address. `[X-Forwarded-For](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-For)`. This is useful for things like updating dynamic DNS services that need your public facing IP address, and because it is plaintext it's easy to shell script by just taking the output from a `curl` command like below.

```sh
echo "$(curl myip.mydomain.com)"
```

### Getting Started

👷 `worker-template` Hello World

A template for kick starting a Cloudflare worker project.

[`index.js`](https://github.com/cloudflare/worker-template/blob/master/index.js) is the content of the Workers script.

### Wrangler (JS)

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

## Cloudflare Native Rust Worker

This has a version of the server made to run as native rust on a Cloudflare worker. Cloudflare uses their own utility to work with their infrastructure called `wrangler`. Start by installing it with `cargo install wrangler`. Then verify it works with `wrangler --version`.

### Getting started

Once `wrangler --verison` verifies it's properly installed run these three commands to start the project.

```sh
wrangler generate --type=rust project_name
cd project name
wrangler build
```

