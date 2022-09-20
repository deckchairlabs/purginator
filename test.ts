import { assertEquals } from "https://deno.land/std@0.156.0/testing/asserts.ts";
import { purge } from "./mod.ts";

Deno.test("it works", async () => {
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

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

  const result = await purge(encoder.encode(css), encoder.encode(html), {
    targets: {
      safari: (13 << 16) | (2 << 8),
    },
  });

  const purged = decoder.decode(result);

  assertEquals(purged.includes(".unused"), false);
});
