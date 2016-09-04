module.exports = (grunt) ->
  grunt.initConfig
    coffee:
      options:
        sourceMap: true
      files:
        expand: true
        cwd: "coffee/"
        src: ['**/*.coffee']
        dest: 'js/'
        ext: '.js'
    uglify:
      options:
        sourceMap: true
        sourceMapIn: (src) ->
          src + '.map'
      files:
        expand: true
        cwd: "js/"
        src: ['**/*.js']
        dest: 'build/'
        ext: '.min.js'
    watch:
      coffee:
        files: ['coffee/**/*.coffee']
        tasks: ['coffee']
      js:
        files: ['js/**/*.js']
        tasks: ['uglify']
      livereload:
        files: ['build/**/*']
        livereload: true

  grunt.loadNpmTasks 'grunt-contrib-coffee'
  grunt.loadNpmTasks 'grunt-contrib-uglify'
  grunt.loadNpmTasks 'grunt-contrib-watch'

  grunt.registerTask 'default', ['coffee', 'uglify']