<template>
  <div class="thread-item">
    <span class="swatch" :style="{backgroundColor: color}">
      <span class="color">{{color}}</span>
    </span>
    <span class="brand">{{ thread.brand }}</span>
    <span class="code">{{ thread.code }}</span>
    <span class="name">{{ thread.name }}</span>
  </div>
</template>

<script>

function toHexCode([r, g, b]) {
  return `#${[r, g, b].map(c => `0${c.toString(16)}`.slice(-2)).join('')}`;
}

export default {
  name: 'ThreadItem',
  props: ['thread'],
  components: {
  },
  computed: {
    // a computed getter
    color() {
      return toHexCode(this.thread.color);
    },
  },
};
</script>

<style>
.thread-item {
  min-width: 15em;
  text-align: start;
  display: grid;
  grid-template-columns: 1fr auto;
  grid-template-rows: 1fr auto auto auto;
  grid-template-areas:
    "swatch swatch"
    "brand code"
    "name name";
}
.swatch {
  grid-area: swatch;
  border: 1px solid black;
  min-width: 5em;
  min-height: 5em;
  display: flex;
  align-items: center;
  justify-content: center;
}
.swatch .color {
  display: none;
}
.swatch:hover .color {
  display: inline;
  background-color: white;
  color: black;
}
.brand {
  grid-area: brand;
}
.code {
  grid-area: code;
}
.name {
  grid-area: name;
}
</style>
