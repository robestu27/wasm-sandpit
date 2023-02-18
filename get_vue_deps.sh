# source this to get vue deps
# curl -L -- follows re-directs

mkdir -p jsdeps/vue3
curl -L --output jsdeps/vue3/vue.global.js https://unpkg.com/vue@3/dist/vue.global.js

mkdir -p jsdeps/primevue/core
curl -L --output jsdeps/primevue/core/core.min.js https://unpkg.com/primevue@^3/core/core.min.js

mkdir -p jsdeps/primevue/treetable
curl -L --output jsdeps/primevue/treetable/treetable.min.js https://unpkg.com/primevue@^3/treetable/treetable.min.js

mkdir -p jsdeps/primevue/column
curl -L --output jsdeps/primevue/column/column.min.js https://unpkg.com/primevue@^3/column/column.min.js

# Themes and styles for PrimeVue
mkdir -p jsdeps/primevue/resources/themes/lara-light-indigo
curl -L --output jsdeps/primevue/resources/themes/lara-light-indigo/theme.css https://unpkg.com/primevue/resources/themes/lara-light-indigo/theme.css
curl -L --output jsdeps/primevue/resources/primevue.min.css https://unpkg.com/primevue/resources/primevue.min.css

mkdir -p jsdeps/primeicons
curl -L --output jsdeps/primeicons/primeicons.css https://unpkg.com/primeicons/primeicons.css

mkdir -p jsdeps/primeicons/fonts
curl -L --output jsdeps/primeicons/fonts/primeicons.woff2 https://unpkg.com/primeicons/fonts/primeicons.woff2
curl -L --output jsdeps/primeicons/fonts/primeicons.woff https://unpkg.com/primeicons/fonts/primeicons.woff
curl -L --output jsdeps/primeicons/fonts/primeicons.ttf https://unpkg.com/primeicons/fonts/primeicons.ttf


mkdir -p jsdeps/primeflex
curl -L --output jsdeps/primeflex/primeflex.min.css https://unpkg.com/primeflex@3.3.0/primeflex.min.css

