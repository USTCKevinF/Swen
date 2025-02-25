import Database from '@tauri-apps/plugin-sql';

// 聊天历史记录接口
export interface ChatHistory {
  id?: number;
  content: string;
  response: string;
  model: string;
  timestamp?: string;
}

// 数据库连接
let db: Database | null = null;

// 初始化数据库
export async function initDatabase() {
  try {
    db = await Database.load('sqlite:chat_history.db');
    
    // 检查旧表是否存在
    const tableExists = await checkTableExists();
    
    if (!tableExists) {
        await createTable();
    }
    console.log('数据库初始化成功');
  } catch (error) {
    console.error('数据库初始化失败:', error);
  }
}

// 检查表是否存在
async function checkTableExists(): Promise<boolean> {
  try {
    if (!db) throw new Error('数据库未初始化');
    
    const result = await db.select(
      `SELECT name FROM sqlite_master WHERE type='table' AND name='chat_history'`
    ) as any[];
    
    return result.length > 0;
  } catch (error) {
    console.error('检查表是否存在失败:', error);
    return false;
  }
}

// 创建表
async function createTable() {
  if (!db) throw new Error('数据库未初始化');
  
  await db.execute(`
    CREATE TABLE IF NOT EXISTS chat_history (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      content TEXT NOT NULL,
      response TEXT NOT NULL,
      model TEXT NOT NULL,
      timestamp TEXT DEFAULT CURRENT_TIMESTAMP
    )
  `);
}

// 保存聊天历史
export async function saveChatHistory(history: ChatHistory) {
  try {
    if (!db) throw new Error('数据库未初始化');
    
    // 如果提供了时间戳，则使用它；否则使用数据库默认的 CURRENT_TIMESTAMP
    if (history.timestamp) {
      await db.execute(
        `INSERT INTO chat_history (content, response, model, timestamp)
         VALUES ($1, $2, $3, $4)`,
        [history.content, history.response, history.model, history.timestamp]
      );
    } else {
      await db.execute(
        `INSERT INTO chat_history (content, response, model)
         VALUES ($1, $2, $3)`,
        [history.content, history.response, history.model]
      );
    }
    return true;
  } catch (error) {
    console.error('保存聊天历史失败:', error);
    return false;
  }
}

// 获取所有聊天历史
export async function getAllChatHistory(): Promise<ChatHistory[]> {
  try {
    if (!db) throw new Error('数据库未初始化');
    
    const result = await db.select(
      'SELECT * FROM chat_history ORDER BY timestamp DESC'
    );
    return result as ChatHistory[];
  } catch (error) {
    console.error('获取聊天历史失败:', error);
    return [];
  }
}

// 获取分页聊天历史
export async function getPaginatedChatHistory(page: number, pageSize: number): Promise<{
  data: ChatHistory[],
  total: number
}> {
  try {
    if (!db) throw new Error('数据库未初始化');
    
    const offset = (page - 1) * pageSize;
    const data = await db.select(
      `SELECT * FROM chat_history ORDER BY timestamp DESC LIMIT $1 OFFSET $2`,
      [pageSize, offset]
    ) as ChatHistory[];
    
    const countResult = await db.select(
      'SELECT COUNT(*) as total FROM chat_history'
    ) as [{total: number}];
    
    return {
      data,
      total: countResult[0].total
    };
  } catch (error) {
    console.error('获取分页聊天历史失败:', error);
    return {
      data: [],
      total: 0
    };
  }
}

// 删除聊天历史
export async function deleteChatHistory(id: number): Promise<boolean> {
  try {
    if (!db) throw new Error('数据库未初始化');
    
    await db.execute(
      'DELETE FROM chat_history WHERE id = $1',
      [id]
    );
    return true;
  } catch (error) {
    console.error('删除聊天历史失败:', error);
    return false;
  }
}

// 清空所有聊天历史
export async function clearAllChatHistory(): Promise<boolean> {
  try {
    if (!db) throw new Error('数据库未初始化');
    
    await db.execute('DELETE FROM chat_history');
    return true;
  } catch (error) {
    console.error('清空聊天历史失败:', error);
    return false;
  }
}

// 提取标题函数，从内容中提取前20个字符作为标题
export function extractTitle(content: string): string {
  const trimmed = content.trim();
  return trimmed.length > 20 ? trimmed.substring(0, 20) + '...' : trimmed;
} 