import { serve } from "bun";

const PORT = 3000;

const server = serve({
  port: PORT,
  fetch(req) {
    return new Response("<h1>I'm other server</h1>", {
      status: 200,
      headers: {
        "Content-Type": "text/html",
      },
    });
  },
});

