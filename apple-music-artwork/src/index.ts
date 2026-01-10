export default {
	async fetch(request, env): Promise<Response> {
		const url = new URL(request.url);

		if (request.method === "POST" && url.pathname === "/upload") {
			const contentType = request.headers.get("content-type") || "";

			if (!contentType.startsWith("image/")) {
				return new Response("Invalid content type", { status: 400 });
			}

			const bytes = await request.arrayBuffer();
			const hashBuffer = await crypto.subtle.digest("SHA-256", bytes);
			const hashArray = [...new Uint8Array(hashBuffer)];
			const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

			const key = `artwork/${hashHex}.png`;

			await env.APPLE_MUSIC_ARTWORK.put(key, new Uint8Array(bytes), {
				httpMetadata: {
					contentType: "image/png",
					cacheControl: "public, max-age=31536000, immutable",
				},
			});

			return Response.json({ 
				hash: hashHex,
				url: `${url.origin}/${key}` 
			});
		}

		if (request.method === "GET" && url.pathname.startsWith("/artwork/")) {
      		const object = await env.APPLE_MUSIC_ARTWORK.get(url.pathname.slice(1));

      		if (!object) {
      		  return new Response("Not found", { status: 404 });
      		}
		
      		return new Response(object.body, {
      		  headers: {
      		    "Content-Type": "image/png",
      		    "Cache-Control": "public, max-age=31536000, immutable"
      		  }
      		});
		}

		return new Response("Not found", { status: 404 });
	},
} satisfies ExportedHandler<Env>;
