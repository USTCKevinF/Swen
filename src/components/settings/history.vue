<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Calendar, Delete } from '@element-plus/icons-vue';
import { ElMessageBox, ElMessage } from 'element-plus';
// @ts-ignore 忽略Vue导入错误
import { ref, reactive, onMounted } from 'vue';
import { initDatabase, getPaginatedChatHistory, deleteChatHistory, ChatHistory, extractTitle } from '../../utils/database';

const { t } = useI18n();

// 历史记录数据
const historyData = ref<ChatHistory[]>([]);

// 分页数据
const pagination = reactive({
  currentPage: 1,
  pageSize: 10,
  total: 0
});

// 当前查看的记录详情
const currentRecord = ref<ChatHistory | null>(null);
const dialogVisible = ref<boolean>(false);
const loading = ref<boolean>(false);

// 从数据库加载历史记录
const loadHistoryData = async () => {
  loading.value = true;
  try {
    const result = await getPaginatedChatHistory(
      pagination.currentPage, 
      pagination.pageSize
    );
    historyData.value = result.data;
    pagination.total = result.total;
  } catch (error) {
    console.error('加载历史记录失败:', error);
    ElMessage({
      type: 'error',
      message: t('settings.history.loadFailed'),
    });
  } finally {
    loading.value = false;
  }
};

// 查看详情
const viewDetail = (record: ChatHistory): void => {
  currentRecord.value = record;
  dialogVisible.value = true;
};

// 删除记录
const deleteRecord = (id: number): void => {
  ElMessageBox.confirm(
    t('settings.history.deleteConfirm'),
    t('common.warning'),
    {
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel'),
      type: 'warning',
    }
  )
    .then(async () => {
      const success = await deleteChatHistory(id);
      if (success) {
        ElMessage({
          type: 'success',
          message: t('settings.history.deleteSuccess'),
        });
        loadHistoryData(); // 重新加载数据
      } else {
        ElMessage({
          type: 'error',
          message: t('settings.history.deleteFailed'),
        });
      }
    })
    .catch(() => {
      // 用户取消删除
    });
};

// 格式化日期
const formatDate = (date: Date | string | null | undefined): string => {
  if (!date) return '';
  const d = new Date(date);
  
  // 格式化为 yyyy-MM-dd HH:mm:ss
  const year = d.getFullYear();
  const month = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  const hours = String(d.getHours()).padStart(2, '0');
  const minutes = String(d.getMinutes()).padStart(2, '0');
  const seconds = String(d.getSeconds()).padStart(2, '0');
  
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
};

// 处理分页
const handleCurrentChange = (val: number): void => {
  pagination.currentPage = val;
  loadHistoryData();
};

// 组件挂载时初始化数据库并加载历史记录
onMounted(async () => {
  await initDatabase();
  loadHistoryData();
});
</script>

<template>
  <div class="history-container">
    <!-- 历史记录列表 -->
    <div class="mb-4 flex-grow overflow-auto">
      <el-table 
        :data="historyData" 
        style="width: 100%" 
        row-key="id"
        stripe
        v-loading="loading"
        @row-click="viewDetail"
        :row-class-name="() => 'cursor-pointer hover:text-blue-500'"
      >
        <el-table-column :label="t('settings.history.question')" width="100">
          <template #default="{ row }">
            <div>
              {{ extractTitle(row.content) }}
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="model" :label="t('settings.history.model')" width="100" />
        <el-table-column prop="timestamp" :label="t('settings.history.time')" width="180">
          <template #default="{ row }">
            <div class="flex items-center">
              <el-icon class="mr-1"><Calendar /></el-icon>
              {{ formatDate(row.timestamp) }}
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('common.operations')" width="50" >
          <template #default="{ row }">
              <el-button type="danger" size="small" circle @click.stop="deleteRecord(row.id)">
                <el-icon><Delete /></el-icon>
              </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 分页 -->
    <div class="pagination-container">
      <el-pagination
        v-model:currentPage="pagination.currentPage"
        :page-size="pagination.pageSize"
        layout="prev, pager, next, jumper"
        :total="pagination.total"
        @current-change="handleCurrentChange"
      />
    </div>

    <!-- 对话详情弹窗 -->
    <el-dialog v-model="dialogVisible" :title="currentRecord ? extractTitle(currentRecord.content) : ''" width="60%" destroy-on-close>
      <template v-if="currentRecord">
        <div class="conversation-detail">
          <!-- 用户问题 -->
          <div class="user-question mb-4 p-4 bg-gray-50 rounded-lg">
            <div class="font-medium mb-2">{{ t('settings.history.question') }}:</div>
            <div>{{ currentRecord.content }}</div>
          </div>
          
          <!-- AI回答 -->
          <div class="ai-response p-4 bg-blue-50 rounded-lg">
            <div class="font-medium mb-2">{{ currentRecord.model }} {{ t('settings.history.response') }}:</div>
            <div class="whitespace-pre-wrap">{{ currentRecord.response }}</div>
          </div>
          
          <!-- 元数据 -->
          <div class="metadata mt-4 text-sm text-gray-500">
            <div>{{ t('settings.history.time') }}: {{ formatDate(currentRecord.timestamp) }}</div>
            <div>{{ t('settings.history.model') }}: {{ currentRecord.model }}</div>
          </div>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.history-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.pagination-container {
  margin-top: auto;
  display: flex;
  justify-content: center;
  padding: 10px 0;
}

.conversation-detail {
  max-height: 60vh;
  overflow-y: auto;
}

:deep(.el-table .cell) {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
:deep(.el-table__header) {
  width: 100% !important;
}
:deep(.el-table__body) {
  width: 100% !important;
}
</style>
