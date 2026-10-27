import express from 'express'
import { exec } from 'node:child_process'
const app = express()
const port = 5173

// Serve static files from the dist directory
app.use('/ui', express.static('dist'))
// Start the server
app.listen(port, () => {
  console.log(`Server listening at http://localhost:${port}`)
})


