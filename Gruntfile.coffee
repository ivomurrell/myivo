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
      all:
        src: ['js/**/*.js']
        dest: 'build/app.min.js'
    sass:
      options:
        sourceMap: true
      files:
        expand: true
        cwd: "sass/"
        src: ['**/*.scss']
        dest: 'css/'
        ext: '.css'
    postcss:
      options:
        map: true
        processors: [
          require('autoprefixer')(browsers: 'last 2 versions')
          require('cssnano')()
        ]
      all:
        src: ['css/**/*.css']
        dest: 'styles/main.min.css'
    processhtml:
      options:
        strip: true
      dist:
        files:
          'dist/index.html': ['index.html']
    watch:
      coffee:
        files: ['coffee/**/*.coffee']
        tasks: ['newer:coffee','newer:uglify:all']
      sass:
        files: ['sass/**/*.scss']
        tasks: ['newer:sass','newer:postcss:all']
      livereload:
        options:
          livereload:true
        files: [
          'build/**/*.js'
          'styles/**/*.css'
          '*.html'
        ]
    newer:
      options:
        override: (detail, include) ->
          if detail.task is 'sass' and detail.path is 'sass/main.scss'
            include true
    clean:
      dist:
        ['dist/*']
    copy:
      dist:
        expand: true
        src: [
          'bower.json'
          'package.json'
          'fonts/**'
          'build/**'
          'styles/**'
        ]
        dest: 'dist/'
          

  grunt.loadNpmTasks 'grunt-contrib-coffee'
  grunt.loadNpmTasks 'grunt-contrib-uglify'
  grunt.loadNpmTasks 'grunt-sass'
  grunt.loadNpmTasks 'grunt-postcss'
  grunt.loadNpmTasks 'grunt-processhtml'
  grunt.loadNpmTasks 'grunt-contrib-watch'
  grunt.loadNpmTasks 'grunt-newer'
  grunt.loadNpmTasks 'grunt-contrib-clean'
  grunt.loadNpmTasks 'grunt-contrib-copy'

  grunt.registerTask 'default', [
    'coffee'
    'uglify'
    'sass'
    'postcss'
  ]
  grunt.registerTask 'production', [
    'clean:dist'
    'default'
    'processhtml:dist'
    'copy:dist'
  ]
