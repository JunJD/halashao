
'use client'
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Comment } from './comment'
import { useComments } from './use-comments'
import { useSearch } from './use-search'
import { AnimatedProgress } from './animated-progress'

export default function CommentSearch() {
  const { comments, isLoading, progress, searchComments } = useComments();
  const { keywords, setKeywords, startPage, setStartPage, maxCount, setMaxCount, handleSearch } = useSearch(searchComments);

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">评论搜索</h1>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
        <div>
          <Label htmlFor="keywords">关键词</Label>
          <Input 
            id="keywords"
            placeholder="输入搜索关键词" 
            value={keywords} 
            onChange={(e) => setKeywords(e.target.value)}
          />
        </div>
        <div>
          <Label htmlFor="startPage">起始页</Label>
          <Input 
            id="startPage"
            type="number" 
            placeholder="输入起始页码" 
            value={startPage} 
            onChange={(e) => setStartPage(Number(e.target.value))}
          />
        </div>
        <div>
          <Label htmlFor="maxCount">最大数量</Label>
          <Input 
            id="maxCount"
            type="number" 
            placeholder="输入最大评论数量" 
            value={maxCount} 
            onChange={(e) => setMaxCount(Number(e.target.value))}
          />
        </div>
      </div>
      <Button onClick={handleSearch} disabled={isLoading} className="mb-4">搜索</Button>
      {isLoading && (
        <div className="mb-4">
          <AnimatedProgress value={progress} />
          <p className="text-center mt-2">正在搜索中...{progress}%</p>
        </div>
      )}
      <div className="space-y-2">
        {comments.map(comment => (
          <Comment key={comment.comment_id} comment={comment} />
        ))}
      </div>
      {comments.length === 0 && !isLoading && (
        <p className="text-center text-gray-500">没有找到匹配的评论</p>
      )}
    </div>
  )
}