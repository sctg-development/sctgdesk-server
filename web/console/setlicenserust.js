/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
* RUN WITH
* cd webconsole
* npx gulp -f setlicenserust.js licensesRust
*/
import fs from 'fs'
import gulp from 'gulp'
import gap from 'gulp-append-prepend'
import gulpif from 'gulp-if'

function checkCopyright(file) {
  const content = fs.readFileSync(file.path, 'utf8')
  return !content.includes('Ronan LE MEILLAT for SCTG Development')
}

const copyrightText = `// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.`

gulp.task('licensesRust', async function () {
  // this is to add Copyright in the production mode for the minified js
  gulp
    .src(['../**/*.rs'], { base: '../' })
    .pipe(gulpif(checkCopyright, gap.prependText(copyrightText)))
    .pipe(gulp.dest('../', { overwrite: true }))
  return
})
