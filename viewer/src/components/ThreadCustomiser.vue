<template>
  <div class="root">
    <div :class="{'bar': true, 'is-fixed': fixed}">
      filter
    </div>
    <div v-if="fixed" class="bar-pad"></div>
  </div>
</template>

<script>

export default {
  name: 'ThreadCustomiser',
  props: ['fixed'],
  components: { },
};
</script>

<style scoped>
.root {
  width: 100vw;
}
.bar {
  display: flex;
  flex-wrap: wrap;
  align-items: stretch;
  align-content: stretch;
}
.bar, .bar-pad {
  height: 3em;
  width: 100vw;
  background-color: white;
}
.is-fixed {
  position: fixed;
}
</style>
