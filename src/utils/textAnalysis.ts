/**
 * 计算文本的复杂度，包括英文单词数和中文字符数
 * @param text 要分析的文本
 * @returns 英文单词数 + 中文字符数的总和
 */
export function calculateTextComplexity(text: string): number {
  // 提取所有英文单词
  const englishWords = text.match(/[a-zA-Z]+/g) || [];
  
  // 提取所有中文字符
  const chineseChars = text.match(/[\u4e00-\u9fa5]/g) || [];
  
  // 英文单词数 + 中文字符数作为总"词"数
  return englishWords.length + chineseChars.length;
}