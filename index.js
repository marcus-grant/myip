addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

function respondWithIP(ip) {
  return new Response(
    ip,
    { headers: {
      'status': 200,
      'content-type': 'text/plain',
    }},
  );
}

function respondWithFail(headers) {
  const errorMsg = 'ERROR: No corresponding header to identify public IP.'
  
  return new Response(
    JSON.stringify(Object.fromEntries(headers)),
    { headers: {
      'status': 406,
      'statusText': errorMsg,
    }}
    
  );
}

/**
 * Respond with hello worker text
 * @param {Request} request
 */
async function handleRequest(request) {
  const headers = new Map(request.headers);
  const headerPriority = [
    'x-forwarded-for',
    'x-real-ip',
    'cf-connecting-ip'
  ];
  let preferredIPHeader;
  for (let h of headerPriority) {
    if (headers.has(h)) {
      // console.log(`currentHeader=${h}, headers.has(h)=${headers.has(h)}`)
      // console.log(`currentHeaderIP=${headers.get(h)}`);
      return respondWithIP(headers.get(h));
    }
  }
  return respondWithFail(headers);
}
