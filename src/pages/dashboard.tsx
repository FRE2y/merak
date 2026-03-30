import { LayoutDashboard, Settings } from 'lucide-react';
import type { ReactNode } from 'react';
import { Button } from '../components/ui/button';

interface DashboardProps {
  children: ReactNode;
}
export default function Dashboard({ children }: DashboardProps) {
  return (
    <div className="flex h-screen w-full bg-background text-foreground">
      <aside className="hidden w-64 flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground md:flex">
        <div className="p-6 text-lg font-semibold">Merak</div>

        <nav className="flex flex-col gap-2 px-4">
          <Button
            variant="ghost"
            className="justify-start text-sidebar-foreground hover:bg-sidebar-accent"
          >
            <LayoutDashboard className="mr-2 h-4 w-4" />
            Dashboard
          </Button>

          <Button
            variant="ghost"
            className="justify-start text-sidebar-foreground hover:bg-sidebar-accent"
          >
            <Settings className="mr-2 h-4 w-4" />
            Settings
          </Button>
        </nav>
      </aside>

      <div className="flex flex-1 flex-col">
        <header className="flex h-14 items-center justify-between border-b border-border bg-background px-6">
          <h1 className="text-lg font-semibold">Dashboard</h1>

          <Button size="sm">Action</Button>
        </header>

        <main className="flex-1 overflow-y-auto p-6">{children}</main>
      </div>
    </div>
  );
}
