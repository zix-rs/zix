#!/bin/bash

rm -rf test_dir
mkdir test_dir
cd test_dir || exit 1

echo "Creando estructura de prueba..."
mkdir subdir1 subdir2 .hidden_dir

touch file1.txt file2.log .hidden_file readme.md

echo "#!/bin/bash\necho 'Script ejecutado'" > script.sh
chmod +x script.sh

echo "{\"key\": \"value\"}" > data.json

echo "[config]\nsetting=true" > settings.config
echo "name = \"test_project\"" > Cargo.toml

echo "# Archivo de configuración" > config.conf

echo "Contenido de prueba" > subdir1/file_in_subdir1.txt
echo "Otro archivo" > subdir2/file_in_subdir2.txt

ln -s subdir1 subdir1_link
ln -s no_existe broken_link
ln -s readme.md readme_link.md
ln -s script.sh script_link.sh

zip -r archive.zip subdir1/ subdir2/

chmod 600 .hidden_file
chmod 700 .hidden_dir
chmod 644 readme.md
chmod 755 subdir2

echo -e "\nEstructura final generada:\n"
"../target/release/zx" -lha

echo -e "\nTest de estructura de directorios completado.\n"
