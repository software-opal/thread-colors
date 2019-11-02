<template>
  <div id="app">
    <ThreadCustomiser fixed="true" />
    <ThreadList :threads="AnchorThreads"/>
  </div>
</template>

<script>
import ThreadList from './components/ThreadList';
import ThreadCustomiser from './components/ThreadCustomiser';
import { convertBrandThreads } from './utils/threads';

const AnchorThreads = convertBrandThreads(require('../../data/brands/anchor.json'));

export default {
  name: 'App',
  components: {
    ThreadList,
    ThreadCustomiser,
  },
  data() {
    return {
      AnchorThreads,
    };
  },
};
</script>

<style>
@import "../node_modules/modern-normalize/modern-normalize.css";

#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
