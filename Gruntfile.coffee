module.exports = (grunt) ->
  grunt.initConfig
    uglify:
      options:
        sourceMap: true
      files:
        expand: true
        cwd: "js/"
        src: ['**/*.js']
        dest: 'build/'
        ext: '.min.js'
    watch:
      js:
        files: ['js/**/*.js']
        tasks: ['uglify']
      livereload:
        files: ['build/**/*']
        livereload: true

  grunt.loadNpmTasks 'grunt-contrib-uglify'
  grunt.loadNpmTasks 'grunt-contrib-watch'

  grunt.registerTask 'default', ['uglify']