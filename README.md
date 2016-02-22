# conf_robber (rust)
# RU: Изменение файла конфигурации 1С.

Перед началом сборки необходимо:

Скачать исходные файлы библиотек:    
- zlib: http://www.zlib.net/ - поместить в каталог cpp_src/zlib/

Установить:
- mingw64: http://sourceforge.net/projects/mingw-w64/
- cmake:   http://www.cmake.org/download/

После установки необходимо изменить значения переменных в cpp_src\build_C_lib.bat.

Планируется:
 - модуль должен уметь изменять структуру конфигурационного файла
 - выполнять локализацию

# EN: Changing the configuration file of 1C

Before building needed:

Download the source files of libraries:
- zlib: http://www.zlib.net/ - unzip in the directory cpp_src/zlib/

To install:
- mingw64: http://sourceforge.net/projects/mingw-w64/
- cmake:   http://www.cmake.org/download/

After installation need change variables in the cpp_src\build_C_lib.bat.

It is planned:
  - The module must be able to change the structure of the configuration file
  - To carry out the localization