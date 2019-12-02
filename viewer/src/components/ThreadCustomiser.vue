<template>
  <div class="root">
    <div :class="{'bar': true, 'is-fixed': fixed}">
      <div class="thread-groups">
        <v-select :clearable="false" :searchable="false" :options="palette" label="name">
          <template slot="option" slot-scope="option">
            <span :style="{backgroundColor: option.css, width: '1em'}"></span>
            {{ option.name }}
          </template>
        </v-select>
      </div>
    </div>
    <div v-if="fixed" class="bar-pad"></div>
  </div>
</template>

<script>
import { ColorPaletteParsedColors } from '../utils/threads.js';

export default {
  name: 'ThreadCustomiser',
  props: ['fixed'],
  components: { },
  data() {
    console.log(ColorPaletteParsedColors);
    return {
      palette: [
        { all: true, css: 'rgba(255,255,255,255)', name: 'Show All' },
        ...ColorPaletteParsedColors,
      ],
    };
  },
};
</script>

<style lang="scss">
/* import 'vue-select/dist/vue-select.css'; */
@import "../../node_modules/vue-select/src/scss/global/_variables.scss";
@import "../../node_modules/vue-select/src/scss/vue-select.scss";
.root {
  width: 100vw;
}
.bar {
  display: flex;
  flex-wrap: wrap;
  align-items: stretch;
  align-content: stretch;
  border-bottom: solid 1px #ddd;

  .thread-groups {

  }
}
.bar, .bar-pad {
  height: 3em;
  width: 100vw;
  background-color: white;
}
.bar.is-fixed {
  position: fixed;
}
</style>
