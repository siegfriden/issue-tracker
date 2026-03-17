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

import { useCreateProject } from '../api/create-project'
import { createProjectSchema } from '../schemas/project'

export function CreateProjectDialog({
  children,
}: {
  children: React.ReactElement
}) {
  const [open, setOpen] = useState(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  const createProject = useCreateProject({
    mutationConfig: {
      onSuccess: () => setOpen(false),
    },
  })

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const fd = new FormData(e.currentTarget)
    const values = {
      name: fd.get('name') as string,
      identifier: fd.get('identifier') as string,
      description: fd.get('description') as string,
      is_public: fd.get('is_public') === 'on',
    }

    const result = createProjectSchema.safeParse(values)
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
    createProject.mutate(result.data)
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger render={children} />
      <DialogContent className="sm:max-w-[480px]">
        <DialogHeader>
          <DialogTitle>Create Project</DialogTitle>
          <DialogDescription>
            Add a new project to your workspace
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit} className="space-y-3">
          <div className="space-y-1.5">
            <Label htmlFor="cp-name">Name</Label>
            <Input id="cp-name" name="name" placeholder="My Project" />
            {errors.name && (
              <p className="text-destructive text-xs">{errors.name}</p>
            )}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="cp-identifier">Identifier</Label>
            <Input
              id="cp-identifier"
              name="identifier"
              placeholder="my-project"
            />
            {errors.identifier && (
              <p className="text-destructive text-xs">{errors.identifier}</p>
            )}
          </div>

          <div className="space-y-1.5">
            <Label htmlFor="cp-description">Description</Label>
            <textarea
              id="cp-description"
              name="description"
              placeholder="Optional description..."
              className="border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 w-full rounded-lg border bg-transparent px-3 py-2 text-sm outline-none focus-visible:ring-3"
              rows={3}
            />
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="cp-public" className="font-normal">
              Public project
            </Label>
            <input
              id="cp-public"
              name="is_public"
              type="checkbox"
              defaultChecked
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
            <Button type="submit" disabled={createProject.isPending}>
              {createProject.isPending ? 'Creating...' : 'Create'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
