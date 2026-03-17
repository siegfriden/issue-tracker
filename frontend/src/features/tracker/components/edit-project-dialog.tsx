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

import { useUpdateProject } from '../api/update-project'
import { updateProjectSchema } from '../schemas/project'
import type { Project } from '../types'

export function EditProjectDialog({
  project,
  children,
}: {
  project: Project
  children: React.ReactElement
}) {
  const [open, setOpen] = useState(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  const updateProject = useUpdateProject({
    mutationConfig: {
      onSuccess: () => setOpen(false),
    },
  })

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    const values = {
      name: fd.get('name') as string,
      description: fd.get('description') as string,
      is_public: fd.get('is_public') === 'on',
    }

    const result = updateProjectSchema.safeParse(values)
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
    updateProject.mutate({ projectId: project.id, data: result.data })
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent className="sm:max-w-[480px]">
        <DialogHeader>
          <DialogTitle>Edit Project</DialogTitle>
          <DialogDescription>Update project settings</DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-3">
          <div className="space-y-1.5">
            <Label htmlFor="ep-name">Name</Label>
            <Input id="ep-name" name="name" defaultValue={project.name} />
            {errors.name && (
              <p className="text-destructive text-xs">{errors.name}</p>
            )}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="ep-description">Description</Label>
            <textarea
              id="ep-description"
              name="description"
              defaultValue={project.description}
              className="border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 w-full rounded-lg border bg-transparent px-3 py-2 text-sm outline-none focus-visible:ring-3"
              rows={3}
            />
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="ep-public" className="font-normal">
              Public project
            </Label>
            <input
              id="ep-public"
              name="is_public"
              type="checkbox"
              defaultChecked={project.is_public}
              className="size-4 rounded"
            />
          </div>

          <DialogFooter>
            <Button
              variant="outline"
              type="button"
              onClick={() => setOpen(false)}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={updateProject.isPending}>
              {updateProject.isPending ? 'Saving...' : 'Save Changes'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
