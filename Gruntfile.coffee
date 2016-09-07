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
    sass:
      options:
        sourceMap: true
      files:
        expand: true
        cwd: "sass/"
        src: ['**/*.scss']
        dest: 'styles/'
        ext: '.css'
    watch:
      coffee:
        files: ['coffee/**/*.coffee']
        tasks: ['coffee']
      uglify:
        files: ['js/**/*.js']
        tasks: ['uglify']
      sass:
        files: ['sass/**/*.scss']
        tasks: ['sass']

  grunt.loadNpmTasks 'grunt-contrib-coffee'
  grunt.loadNpmTasks 'grunt-contrib-uglify'
  grunt.loadNpmTasks 'grunt-sass'
  grunt.loadNpmTasks 'grunt-contrib-watch'

  grunt.registerTask 'default', ['coffee', 'uglify', 'sass']