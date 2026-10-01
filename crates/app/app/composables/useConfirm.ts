import { ConfirmModal } from '#components'

export function useConfirm() {
  const overlay = useOverlay()
  const modal = overlay.create(ConfirmModal)

  return async (props: InstanceType<typeof ConfirmModal>['$props']) => {
    const confirmed = await modal.open(props).result
    return confirmed === true
  }
}
