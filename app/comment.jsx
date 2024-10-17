'use client'
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
// import { Comment as CommentType } from './mock-data'

// interface CommentProps {
//   comment: CommentType;
//   depth?: number;
// }

export const Comment = ({ comment, depth = 0 }) => {
  return (
    <Card className={`mb-2 ${depth > 0 ? 'ml-4 border-l-2 border-gray-200' : ''}`}>
      <CardContent className="py-2 px-3">
        <div className="flex items-start space-x-2">
          <Avatar className="w-8 h-8">
            <AvatarImage src={comment.avatar} alt={comment.nickname} />
            <AvatarFallback>{comment.nickname[0]}</AvatarFallback>
          </Avatar>
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between">
              <h4 className="text-sm font-semibold truncate">{comment.nickname}</h4>
              <span className="text-xs text-gray-500">{new Date(comment.create_time).toLocaleString()}</span>
            </div>
            <p className="text-sm mt-1 break-words">{comment.content}</p>
            <div className="mt-1 flex items-center justify-between text-xs text-gray-500">
              <span>点赞: {comment.like_count}</span>
              <Button 
                variant="link" 
                className="p-0 h-auto text-xs font-normal text-blue-500 hover:text-blue-700"
                onClick={() => window.open(`/notes/${comment.note_id}#comment-${comment.comment_id}`, '_blank')}
              >
                查看原文
              </Button>
            </div>
          </div>
        </div>
      </CardContent>
      {comment.replies && comment.replies.map(reply => (
        <Comment key={reply.comment_id} comment={reply} depth={depth + 1} />
      ))}
    </Card>
  )
}