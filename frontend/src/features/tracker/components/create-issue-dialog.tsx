import { useState } from 'react'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

import { useCreateIssue } from '../api/create-issue'
import { createIssueSchema } from '../schemas/issue'

export function CreateIssueDialog({
  projectId,
  children,
}: {
  projectId: string
  children: React.ReactElement
}) {
  const [open, setOpen] = useState(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  const createIssue = useCreateIssue({
    projectId,
    mutationConfig: {
      onSuccess: () => setOpen(false),
    },
  })

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    const values = {
      subject: fd.get('subject') as string,
      description: fd.get('description') as string,
      issue_type: fd.get('issue_type') as string,
      priority: fd.get('priority') as string,
      due_date: (fd.get('due_date') as string) || undefined,
    }

    const result = createIssueSchema.safeParse(values)
    if (!result.success) {
      const fieldErrors: Record<string, string> = {}
      for (const issue of result.error.issues) {
        const path = issue.path.join('.')
        if (!fieldErrors[path]) fieldErrors[path] = issue.message
      }
      setErrors(fieldErrors)
      return
    }

    setErrors({})
    createIssue.mutate({ projectId, data: result.data })
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent className="sm:max-w-[520px]">
        <DialogHeader>
          <DialogTitle>Create Issue</DialogTitle>
          <DialogDescription>
            Report a bug, request a feature, or create a task
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-3">
          <div className="space-y-1.5">
            <Label htmlFor="ci-subject">Subject</Label>
            <Input id="ci-subject" name="subject" placeholder="Issue title" />
            {errors.subject && (
              <p className="text-destructive text-xs">{errors.subject}</p>
            )}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="ci-description">Description</Label>
            <textarea
              id="ci-description"
              name="description"
              placeholder="Describe the issue..."
              className="border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 w-full rounded-lg border bg-transparent px-3 py-2 text-sm outline-none focus-visible:ring-3"
              rows={3}
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1.5">
              <Label htmlFor="ci-type">Type</Label>
              <select
                id="ci-type"
                name="issue_type"
                defaultValue="task"
                className="border-input bg-background h-9 w-full rounded-lg border px-3 text-sm"
              >
                <option value="bug">Bug</option>
                <option value="feature">Feature</option>
                <option value="task">Task</option>
                <option value="support">Support</option>
              </select>
            </div>

            <div className="space-y-1.5">
              <Label htmlFor="ci-priority">Priority</Label>
              <select
                id="ci-priority"
                name="priority"
                defaultValue="normal"
                className="border-input bg-background h-9 w-full rounded-lg border px-3 text-sm"
              >
                <option value="low">Low</option>
                <option value="normal">Normal</option>
                <option value="high">High</option>
                <option value="urgent">Urgent</option>
                <option value="immediate">Immediate</option>
              </select>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1.5">
              <Label htmlFor="ci-assignee">Assignee</Label>
              <Input
                id="ci-assignee"
                name="assignee_id"
                placeholder="User ID (optional)"
              />
            </div>

            <div className="space-y-1.5">
              <Label htmlFor="ci-due">Due Date</Label>
              <Input id="ci-due" name="due_date" type="date" />
            </div>
          </div>

          <DialogFooter>
            <Button
              variant="outline"
              type="button"
              onClick={() => setOpen(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={createIssue.isPending}>
              {createIssue.isPending ? 'Creating...' : 'Create Issue'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
