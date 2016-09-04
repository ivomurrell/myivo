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

  grunt.loadNpmTasks 'grunt-contrib-coffee'
  grunt.loadNpmTasks 'grunt-contrib-uglify'

  grunt.registerTask 'default', ['coffee', 'uglify']