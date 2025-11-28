/* STM32L432KC: 256 KB Flash, 64 KB RAM */
/* Uses addresses from the ST datasheet for STM32L4.:contentReference[oaicite:7]{index=7} */

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM   : ORIGIN = 0x20000000, LENGTH = 64K
}

