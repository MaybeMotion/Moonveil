import AppKit
let directory=URL(fileURLWithPath:CommandLine.arguments[1],isDirectory:true)
try FileManager.default.createDirectory(at:directory,withIntermediateDirectories:true)
for size in [16,32,64,128,256,512,1024] {
 let s=CGFloat(size)
 let image=NSImage(size:NSSize(width:s,height:s))
 image.lockFocus()
 NSColor(calibratedRed:0.065,green:0.055,blue:0.12,alpha:1).setFill()
 NSBezierPath(roundedRect:NSRect(x:0,y:0,width:s,height:s),xRadius:s*0.22,yRadius:s*0.22).fill()
 let gradient=NSGradient(starting:NSColor(calibratedRed:0.28,green:0.17,blue:0.38,alpha:1),ending:NSColor(calibratedRed:0.08,green:0.07,blue:0.15,alpha:1))!
 gradient.draw(in:NSBezierPath(roundedRect:NSRect(x:s*0.04,y:s*0.04,width:s*0.92,height:s*0.92),xRadius:s*0.20,yRadius:s*0.20),angle:70)
 let crescent=NSBezierPath()
 crescent.windingRule = .evenOdd
 crescent.appendOval(in:NSRect(x:s*0.21,y:s*0.19,width:s*0.59,height:s*0.65))
 crescent.appendOval(in:NSRect(x:s*0.36,y:s*0.31,width:s*0.54,height:s*0.57))
 NSColor(calibratedRed:0.97,green:0.68,blue:0.79,alpha:1).setFill()
 crescent.fill()
 // Mask the outer part of the subtracting circle using a second path with an intersection clip.
 NSGraphicsContext.saveGraphicsState()
 NSBezierPath(ovalIn:NSRect(x:s*0.21,y:s*0.19,width:s*0.59,height:s*0.65)).addClip()
 gradient.draw(in:NSBezierPath(ovalIn:NSRect(x:s*0.36,y:s*0.31,width:s*0.54,height:s*0.57)),angle:70)
 NSGraphicsContext.restoreGraphicsState()
 func sparkle(_ x:CGFloat,_ y:CGFloat,_ r:CGFloat){
  let p=NSBezierPath();p.move(to:NSPoint(x:x,y:y+r));p.line(to:NSPoint(x:x+r*0.25,y:y+r*0.25));p.line(to:NSPoint(x:x+r,y:y));p.line(to:NSPoint(x:x+r*0.25,y:y-r*0.25));p.line(to:NSPoint(x:x,y:y-r));p.line(to:NSPoint(x:x-r*0.25,y:y-r*0.25));p.line(to:NSPoint(x:x-r,y:y));p.line(to:NSPoint(x:x-r*0.25,y:y+r*0.25));p.close();NSColor(calibratedRed:0.98,green:0.83,blue:0.58,alpha:1).setFill();p.fill()
 }
 sparkle(s*0.69,s*0.62,s*0.11);sparkle(s*0.77,s*0.43,s*0.045);sparkle(s*0.27,s*0.77,s*0.035)
 image.unlockFocus()
 let rep=NSBitmapImageRep(data:image.tiffRepresentation!)!
 let png=rep.representation(using:.png,properties:[:])!
 try png.write(to:directory.appendingPathComponent("\(size).png"))
}
