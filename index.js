addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

function respondWithIP(ip) {
  return new Response(
    ip,
    { 'status': 200,
      'statusText': 'OK',
      headers: {
        "content-type": "text/plain;charset=UTF-8",
      },
    }
  );
}

function respondWithFail(headers) {
  const errorMsg = 'ERROR: No corresponding header to identify public IP.'
  
  // TODO headers should be on same depth as status and text
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
  for (let h of headerPriority) {
    if (headers.has(h)) {
      return respondWithIP(headers.get(h));
    }
  }
  return respondWithFail(headers);
}
