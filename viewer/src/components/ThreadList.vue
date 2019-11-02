<template>
  <div class="thread-list">
    <div class="thread-list-item-wrapper" v-for="thread in threads" :key="thread.id">
      <ThreadItem :thread="thread" />
    </div>
  </div>
</template>

<script>

import ThreadItem from './ThreadItem';

export default {
  name: 'ThreadList',
  props: ['threads'],
  components: {
    ThreadItem,
  },
};
</script>

<style>
.thread-list {
  display: flex;
  flex-wrap: wrap;
  align-items: stretch;
  align-content: stretch;
}
.thread-list-item-wrapper {
  display: flex;
  align-items: stretch;
  align-content: stretch;
  flex-grow: 1;
  padding: .5em;
}
</style>
