# Created by: Olivier Duchateau
# $FreeBSD$

PORTNAME=	lxqt-openssh-askpass
PORTVERSION=	0.15.0
CATEGORIES=	security
MASTER_SITES=	LXQT

MAINTAINER=	jsm@FreeBSD.org
COMMENT=	LXQt GUI dialog for OpenSSH authentication

LICENSE=	LGPL21+

USES=	cmake compiler:c++11-lang gettext-runtime kde:5 lxqt \
	pkgconfig qt:5 tar:xz gnome
USE_QT=	buildtools_build qmake_build core dbus gui linguisttools \
	svg widgets x11extras xml
USE_KDE=	windowsystem
USE_LXQT=	buildtools lxqt
USE_GNOME=	glib20

.include <bsd.port.mk>
