MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
    RAM   : ORIGIN = 0x20000000, LENGTH = 96K
}

/* Add a section for defmt logs */
SECTIONS
{
    /* Keep defmt metadata for decoding */
    .defmt :
    {
        KEEP(*(.defmt))
    } > FLASH

    .defmt.end :
    {
        KEEP(*(.defmt.end))
    } > FLASH
}
