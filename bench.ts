import { purge } from "./mod.ts";

Deno.bench("simple purge", async () => {
  const encoder = new TextEncoder();

  const css = `
    body { background-color: red; }
    
    .foo {
      color: blue;
    }

    .unused {
      color: green;
    }

    .foo {
      & a {
        color: red;
      }
    }

    a {
      text-decoration: none;
    }
    
    a:hover {
      text-decoration: underline;
    }
  `;

  const html = `
    <html>
      <head>
        <title>Test</title>
      </head>
      <body>
        <div class="foo">
          <a href="#">Hello</a>
        </div>
      </body>
    </html>
  `;

  await purge(encoder.encode(css), encoder.encode(html), {
    targets: {
      safari: (13 << 16) | (2 << 8),
    },
  });
});
