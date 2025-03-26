import { serve } from "bun";

const PORT = 3000;

var counter = 0;

const server = serve({
  port: PORT,
  fetch(req) {
     console.log("req!");
     counter += 1;
    return new Response(`<h1>I'm other server, magic ${counter}</h1>`, {
      status: 200,
      headers: {
        "Content-Type": "text/html",
      },
    });
  },
});

