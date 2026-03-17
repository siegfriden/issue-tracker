import { UserPlus } from 'lucide-react'
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

import { useAddMember } from '../api/add-member'
import { addMemberSchema } from '../schemas/member'

export function AddMemberDialog({
  projectId,
  children,
}: {
  projectId: string
  children: React.ReactElement
}) {
  const [open, setOpen] = useState(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  const addMember = useAddMember({
    projectId,
    mutationConfig: {
      onSuccess: () => setOpen(false),
    },
  })

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    const values = {
      user_id: fd.get('user_id') as string,
      role: fd.get('role') as string,
    }

    const result = addMemberSchema.safeParse(values)
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
    addMember.mutate({ projectId, data: result.data })
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent className="sm:max-w-[440px]">
        <DialogHeader>
          <DialogTitle>Add Member</DialogTitle>
          <DialogDescription>Invite a user to this project</DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-3">
          <div className="space-y-1.5">
            <Label htmlFor="am-user">User ID</Label>
            <Input id="am-user" name="user_id" placeholder="User UUID" />
            {errors.user_id && (
              <p className="text-destructive text-xs">{errors.user_id}</p>
            )}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="am-role">Role</Label>
            <select
              id="am-role"
              name="role"
              defaultValue="member"
              className="border-input bg-background h-9 w-full rounded-lg border px-3 text-sm"
            >
              <option value="admin">Admin</option>
              <option value="member">Member</option>
              <option value="viewer">Viewer</option>
            </select>
          </div>

          <DialogFooter>
            <Button
              variant="outline"
              type="button"
              onClick={() => setOpen(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={addMember.isPending}>
              <UserPlus className="size-3.5" data-icon="inline-start" />
              {addMember.isPending ? 'Adding...' : 'Add Member'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
