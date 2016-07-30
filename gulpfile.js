const gulp = require('gulp');
const sass = require('gulp-sass');
const moduleImporter = require('sass-module-importer');
const sourcemaps = require('gulp-sourcemaps');
const autoprefixer = require('gulp-autoprefixer');


/// Stylesheet processing

var sassInput = './style/**/*.scss';
var sassOutput = './public/style';
var sassOptions = {
    importer: moduleImporter()
};

gulp.task('sass', function() {
    return gulp.src(sassInput)
        .pipe(sourcemaps.init())
        .pipe(sass(sassOptions).on('error', sass.logError))
        .pipe(sourcemaps.write(undefined, { sourceRoot: null })) // < fix for bug in 1.x sourcemaps
        .pipe(autoprefixer())
        .pipe(gulp.dest(sassOutput));
});

gulp.task('sass-watch', function() {
    return gulp.watch(sassInput, ['default']);
});

gulp.task('sass-prod', function() {
    return gulp
        .src(sassInput)
        .pipe(sass({ outputStyle: 'compressed' }))
        .pipe(autoprefixer())
        .pipe(gulp.dest(sassOutput));
});


/// Entry points

gulp.task('default', ['sass']);
gulp.task('watch', ['sass-watch']);
gulp.task('prod', ['sass-prod']);
