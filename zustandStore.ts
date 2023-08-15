import {create} from 'zustand'

interface AccountState {
  displayName: string,
  setDisplayName: (to: string) => void
}

const accountStore = create<AccountState>()((set) => ({
  displayName: "",
  setDisplayName: (to) => set((state) => ({ displayName: to }))
}))