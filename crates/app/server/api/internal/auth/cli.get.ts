// Legacy verification URI, kept so links from older servers still land on the approval page.
export default defineEventHandler((event) => {
  const userCode = getQuery(event).user_code
  return sendRedirect(event, userCode ? `/device?user_code=${encodeURIComponent(String(userCode))}` : '/device')
})
