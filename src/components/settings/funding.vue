<template>
    <div class="h-full flex flex-col justify-center items-center">
        <h1 class="text-sm text-center mb-4">{{ t('settings.funding.description') }}</h1>
        <img src="../../assets/images/rewardcode.jpg" class="w-80 h-80 mx-auto">
    </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<style scoped>
</style>