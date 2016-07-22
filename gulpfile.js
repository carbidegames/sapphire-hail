var gulp = require('gulp');
var sass = require('gulp-sass');
var moduleImporter = require('sass-module-importer');

gulp.task('sass', function() {
    return gulp.src('./style/**/*.scss')
        .pipe(sass({ importer: moduleImporter() }).on('error', sass.logError))
        .pipe(gulp.dest('./public/style'));
});

gulp.task('default', ['sass']);
