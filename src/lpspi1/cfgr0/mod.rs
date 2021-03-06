#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `HREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRENR {
    #[doc = "Host request is disabled"]
    HREN_0,
    #[doc = "Host request is enabled"]
    HREN_1,
}
impl HRENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRENR::HREN_0 => false,
            HRENR::HREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRENR {
        match value {
            false => HRENR::HREN_0,
            true => HRENR::HREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HREN_0`"]
    #[inline]
    pub fn is_hren_0(&self) -> bool {
        *self == HRENR::HREN_0
    }
    #[doc = "Checks if the value of the field is `HREN_1`"]
    #[inline]
    pub fn is_hren_1(&self) -> bool {
        *self == HRENR::HREN_1
    }
}
#[doc = "Possible values of the field `HRPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRPOLR {
    #[doc = "Active low"]
    HRPOL_0,
    #[doc = "Active high"]
    HRPOL_1,
}
impl HRPOLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRPOLR::HRPOL_0 => false,
            HRPOLR::HRPOL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRPOLR {
        match value {
            false => HRPOLR::HRPOL_0,
            true => HRPOLR::HRPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRPOL_0`"]
    #[inline]
    pub fn is_hrpol_0(&self) -> bool {
        *self == HRPOLR::HRPOL_0
    }
    #[doc = "Checks if the value of the field is `HRPOL_1`"]
    #[inline]
    pub fn is_hrpol_1(&self) -> bool {
        *self == HRPOLR::HRPOL_1
    }
}
#[doc = "Possible values of the field `HRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSELR {
    #[doc = "Host request input is the LPSPI_HREQ pin"]
    HRSEL_0,
    #[doc = "Host request input is the input trigger"]
    HRSEL_1,
}
impl HRSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HRSELR::HRSEL_0 => false,
            HRSELR::HRSEL_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRSELR {
        match value {
            false => HRSELR::HRSEL_0,
            true => HRSELR::HRSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRSEL_0`"]
    #[inline]
    pub fn is_hrsel_0(&self) -> bool {
        *self == HRSELR::HRSEL_0
    }
    #[doc = "Checks if the value of the field is `HRSEL_1`"]
    #[inline]
    pub fn is_hrsel_1(&self) -> bool {
        *self == HRSELR::HRSEL_1
    }
}
#[doc = "Possible values of the field `CIRFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRFIFOR {
    #[doc = "Circular FIFO is disabled"]
    CIRFIFO_0,
    #[doc = "Circular FIFO is enabled"]
    CIRFIFO_1,
}
impl CIRFIFOR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CIRFIFOR::CIRFIFO_0 => false,
            CIRFIFOR::CIRFIFO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIRFIFOR {
        match value {
            false => CIRFIFOR::CIRFIFO_0,
            true => CIRFIFOR::CIRFIFO_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIRFIFO_0`"]
    #[inline]
    pub fn is_cirfifo_0(&self) -> bool {
        *self == CIRFIFOR::CIRFIFO_0
    }
    #[doc = "Checks if the value of the field is `CIRFIFO_1`"]
    #[inline]
    pub fn is_cirfifo_1(&self) -> bool {
        *self == CIRFIFOR::CIRFIFO_1
    }
}
#[doc = "Possible values of the field `RDMO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMOR {
    #[doc = "Received data is stored in the receive FIFO as in normal operations"]
    RDMO_0,
    #[doc = "Received data is discarded unless the Data Match Flag (DMF) is set"]
    RDMO_1,
}
impl RDMOR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RDMOR::RDMO_0 => false,
            RDMOR::RDMO_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMOR {
        match value {
            false => RDMOR::RDMO_0,
            true => RDMOR::RDMO_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDMO_0`"]
    #[inline]
    pub fn is_rdmo_0(&self) -> bool {
        *self == RDMOR::RDMO_0
    }
    #[doc = "Checks if the value of the field is `RDMO_1`"]
    #[inline]
    pub fn is_rdmo_1(&self) -> bool {
        *self == RDMOR::RDMO_1
    }
}
#[doc = "Values that can be written to the field `HREN`"]
pub enum HRENW {
    #[doc = "Host request is disabled"]
    HREN_0,
    #[doc = "Host request is enabled"]
    HREN_1,
}
impl HRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRENW::HREN_0 => false,
            HRENW::HREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRENW<'a> {
    w: &'a mut W,
}
impl<'a> _HRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host request is disabled"]
    #[inline]
    pub fn hren_0(self) -> &'a mut W {
        self.variant(HRENW::HREN_0)
    }
    #[doc = "Host request is enabled"]
    #[inline]
    pub fn hren_1(self) -> &'a mut W {
        self.variant(HRENW::HREN_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRPOL`"]
pub enum HRPOLW {
    #[doc = "Active low"]
    HRPOL_0,
    #[doc = "Active high"]
    HRPOL_1,
}
impl HRPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRPOLW::HRPOL_0 => false,
            HRPOLW::HRPOL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _HRPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active low"]
    #[inline]
    pub fn hrpol_0(self) -> &'a mut W {
        self.variant(HRPOLW::HRPOL_0)
    }
    #[doc = "Active high"]
    #[inline]
    pub fn hrpol_1(self) -> &'a mut W {
        self.variant(HRPOLW::HRPOL_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HRSEL`"]
pub enum HRSELW {
    #[doc = "Host request input is the LPSPI_HREQ pin"]
    HRSEL_0,
    #[doc = "Host request input is the input trigger"]
    HRSEL_1,
}
impl HRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRSELW::HRSEL_0 => false,
            HRSELW::HRSEL_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host request input is the LPSPI_HREQ pin"]
    #[inline]
    pub fn hrsel_0(self) -> &'a mut W {
        self.variant(HRSELW::HRSEL_0)
    }
    #[doc = "Host request input is the input trigger"]
    #[inline]
    pub fn hrsel_1(self) -> &'a mut W {
        self.variant(HRSELW::HRSEL_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CIRFIFO`"]
pub enum CIRFIFOW {
    #[doc = "Circular FIFO is disabled"]
    CIRFIFO_0,
    #[doc = "Circular FIFO is enabled"]
    CIRFIFO_1,
}
impl CIRFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIRFIFOW::CIRFIFO_0 => false,
            CIRFIFOW::CIRFIFO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIRFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _CIRFIFOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIRFIFOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Circular FIFO is disabled"]
    #[inline]
    pub fn cirfifo_0(self) -> &'a mut W {
        self.variant(CIRFIFOW::CIRFIFO_0)
    }
    #[doc = "Circular FIFO is enabled"]
    #[inline]
    pub fn cirfifo_1(self) -> &'a mut W {
        self.variant(CIRFIFOW::CIRFIFO_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDMO`"]
pub enum RDMOW {
    #[doc = "Received data is stored in the receive FIFO as in normal operations"]
    RDMO_0,
    #[doc = "Received data is discarded unless the Data Match Flag (DMF) is set"]
    RDMO_1,
}
impl RDMOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMOW::RDMO_0 => false,
            RDMOW::RDMO_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMOW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is stored in the receive FIFO as in normal operations"]
    #[inline]
    pub fn rdmo_0(self) -> &'a mut W {
        self.variant(RDMOW::RDMO_0)
    }
    #[doc = "Received data is discarded unless the Data Match Flag (DMF) is set"]
    #[inline]
    pub fn rdmo_1(self) -> &'a mut W {
        self.variant(RDMOW::RDMO_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline]
    pub fn hren(&self) -> HRENR {
        HRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline]
    pub fn hrpol(&self) -> HRPOLR {
        HRPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline]
    pub fn hrsel(&self) -> HRSELR {
        HRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline]
    pub fn cirfifo(&self) -> CIRFIFOR {
        CIRFIFOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline]
    pub fn rdmo(&self) -> RDMOR {
        RDMOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline]
    pub fn hren(&mut self) -> _HRENW {
        _HRENW { w: self }
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline]
    pub fn hrpol(&mut self) -> _HRPOLW {
        _HRPOLW { w: self }
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline]
    pub fn hrsel(&mut self) -> _HRSELW {
        _HRSELW { w: self }
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline]
    pub fn cirfifo(&mut self) -> _CIRFIFOW {
        _CIRFIFOW { w: self }
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline]
    pub fn rdmo(&mut self) -> _RDMOW {
        _RDMOW { w: self }
    }
}
