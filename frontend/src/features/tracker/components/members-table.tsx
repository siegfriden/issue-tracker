import { UserPlus } from 'lucide-react'

import { Button } from '@/components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

import { useMembers } from '../api/get-members'
import { AddMemberDialog } from './add-member-dialog'
import { formatDate, initials } from './utils'

export function MembersTable({ projectId }: { projectId: string }) {
  const { data, isLoading } = useMembers({ projectId })

  const members = data?.data ?? []

  return (
    <div className="flex flex-col gap-4">
      <div className="flex justify-end">
        <AddMemberDialog projectId={projectId}>
          <Button>
            <UserPlus className="size-3.5" data-icon="inline-start" />
            Add Member
          </Button>
        </AddMemberDialog>
      </div>

      <div className="overflow-hidden rounded-xl border bg-white">
        <Table>
          <TableHeader>
            <TableRow className="bg-muted/50">
              <TableHead className="w-full">User</TableHead>
              <TableHead className="w-full">Email</TableHead>
              <TableHead className="w-[120px]">Role</TableHead>
              <TableHead className="w-[140px]">Joined</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {isLoading &&
              Array.from({ length: 3 }).map((_, i) => (
                <TableRow key={i}>
                  <TableCell colSpan={4}>
                    <div className="bg-muted h-4 w-full animate-pulse rounded" />
                  </TableCell>
                </TableRow>
              ))}

            {!isLoading && members.length === 0 && (
              <TableRow>
                <TableCell
                  colSpan={4}
                  className="text-muted-foreground py-8 text-center"
                >
                  No members found
                </TableCell>
              </TableRow>
            )}

            {members.map((member) => (
              <TableRow key={member.user_id}>
                <TableCell>
                  <div className="flex items-center gap-2">
                    <div className="bg-muted text-muted-foreground flex size-7 items-center justify-center rounded-full text-[11px] font-semibold">
                      {initials(member.user?.display_name ?? '?')}
                    </div>
                    <span className="text-foreground text-[13px] font-medium">
                      {member.user?.display_name ?? 'Unknown'}
                    </span>
                  </div>
                </TableCell>
                <TableCell className="text-muted-foreground text-[13px]">
                  {member.user?.email ?? '-'}
                </TableCell>
                <TableCell>
                  <span className="bg-secondary text-secondary-foreground rounded-full px-2 py-0.5 text-[11px] font-medium capitalize">
                    {member.role}
                  </span>
                </TableCell>
                <TableCell className="text-muted-foreground text-[13px]">
                  {formatDate(member.joined_at)}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  )
}
